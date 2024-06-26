use scrypto::prelude::*;

#[derive(ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe)]
pub enum RiskAppetite {
    Low,
    Medium,
    High,
}
#[derive(ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe)]
pub struct UserPreference {
    // the financial goal of the user
    finance_goal: String,
    // the risk appetite of the user (low, medium, high)
    risk_appetite: RiskAppetite,
    // the minimum range of time (in Days) the user is willing to invest
    yield_duration: u64,
    // the minimum yield (APY) the user is willing to accept
    min_yield: Decimal,
}

#[blueprint]
mod companion {
    use std::ptr::null;

    struct Companion {
        // Total amount of XRD  tokens invested by users in the vault
        total_invested_amount: Vault,
        // Percentage  fee charged on each investment
        platform_fee: Decimal,
        // Total amount of XRD tokens collected as fees
        total_fees_collected: Vault,
        // Auto invest preferences of users
        auto_invest: bool,
        // user badge resource
        investor_badge: ResourceAddress,
    }

    impl Companion {
        // Implement the functions and methods which will manage those resources and data

        // This is a function, and can be called directly on the blueprint once deployed
        pub fn instantiate(platform_fee: Decimal) -> (ComponentAddress, Bucket, ResourceAddress) {
            // Create an admin badge resource
            let admin_badge = ResourceBuilder::new_fungible()
                .metadata("name", "Admin Badge")
                .metadata("description", "Admin Badge")
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1);

            // Create an investor badge resource
            let investor_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Investor Badge")
                .metadata("description", "Investor Badge")
                .create_with_no_initial_supply();

            // create an access rule for the admin badge resource
            let admin_badge_access_rule = AccessRules::new()
                .method(
                    "total_fees_collected",
                    rule!(require(admin_badge.resource_address())),
                    AccessRule::DenyAll,
                )
                .method(
                    "withdraw_fees",
                    rule!(require(admin_badge.resource_address())),
                    AccessRule::DenyAll,
                )
                .method(
                    "change_platform_fee",
                    rule!(require(admin_badge.resource_address())),
                    AccessRule::DenyAll,
                )
                .method(
                    "total_invested_amount",
                    rule!(require(admin_badge.resource_address())),
                    AccessRule::DenyAll,
                )
                .default(rule!(allow_all), AccessRule::DenyAll);

            let mut component = Self {
                total_invested_amount: Vault::new(RADIX_TOKEN),
                total_fees_collected: Vault::new(RADIX_TOKEN),
                platform_fee,
                auto_invest: false,
                investor_badge,
            }
            .instantiate();
            component.add_access_check(admin_badge_access_rule);
            let component = component.globalize();

            (component, admin_badge, investor_badge)
        }

        // This function let user create  their investment preferences
        pub fn create_preference(&self, preferences: UserPreference) -> Bucket {
            let _newPreference = UserPreference { ..preferences };

            // publish the user preferences to the web-2 database for the recommendation engine to use
            print!("User preferences: {:?}", stringify!(newPreference));

            // mint the investor badge
            let manager = borrow_resource_manager!(self.investor_badge);
            let new_investor_badge: Bucket = manager.mint(1);

            // return the investor badge into the user's vault
            return new_investor_badge;
        }

        // This function let user invest in the vault
        pub fn invest(&mut self, mut amount: Bucket) {
            // check if the user has created their preferences
            require(self.investor_badge);
            // remove the platform fee from the total amount invested
            let our_fee: Bucket = amount.take(amount.amount() * self.platform_fee);
            let remainder: Bucket = amount.take(amount.amount() - our_fee.amount());
            self.total_fees_collected.put(our_fee);

            // add the amount invested to the total amount invested
            self.total_invested_amount.put(remainder);
        }

        // This function let user to enable the system to invest on their behalf based on their preferences
        pub fn enable_auto_invest(&mut self) {
            // check if the user has created their preferences
            require(self.investor_badge);
            // else, enable auto invest
            self.auto_invest = true;
        }

        // This function let user withdraw their investment
        pub fn withdraw(&mut self, amount: Decimal) {
            // check if the user has an active investment
            require(self.investor_badge);
            // if not, throw an error
            // else, withdraw the investment
            self.total_invested_amount.take(amount);
        }

        // This function let user to disable the system to invest on their behalf based on their preferences
        pub fn disable_auto_invest(&mut self) {
            // check if the user has created their preferences
            require(self.investor_badge);
            // if not, throw an error
            // else, disable auto invet
            self.auto_invest = false;
        }

        // this platform let admin see the total amount of fees collected
        pub fn total_fees_collected(&mut self) -> Decimal {
            let total_fees =self.total_fees_collected.amount();
            return total_fees;
        }

        // this function let admin to withdraw the fees collected
        pub fn withdraw_fees(&mut self) {
            self.total_fees_collected.take_all();
        }

        // this function let admin to change the platform fee
        pub fn change_platform_fee(&mut self, platform_fee: Decimal) {
            // check if the platform fee is between 0 and 1
            // if not, throw an error
            assert!(platform_fee >= dec!("0") && platform_fee <= dec!("1"), "Platform fee must be between 0 and 1");
            // else, change the platform fee
            self.platform_fee = platform_fee;
        }

        // this function let admin see the total amount of XRD tokens invested by users in the vault
        pub fn total_invested_amount(&mut self) {
            self.total_invested_amount.amount();
        }
    }
}
