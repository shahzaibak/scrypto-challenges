![](./images/trading.jpg)

Trading dApp is a proof-of-concept protocol of a 'simulated' trading application operating on crypto tokens pair. It has been  built on the Radix ledger using v0.4.1 of Scrypto: the smart contract language of the Radix ledger.   
  
# Abstract 

The Trading dApp is a simulated simple trading application built to be used from the Portfolio dApp blueprint.

It contains a fixed number of vaults, exactly four vaults for trading on the pairs xrd/btc, xrd/eth, xrd/leo and a function that simulates the movements in their prices as epoch advances.

Trading dApp has only some simple rules:
- buy_generic(amount, ResourceAddress) -> a buy order is issued using for the amount specified and the resource address is the token that will be bought,
- sell_generic(bucket) -> a sell order is issued using the amount in the bucket and its  resource address


# Integration Test

The portfolio_dapp.sh is a bash script that contains all the functions and methods tested, also buy/sell methods of this simple blueprint

# Unit Test

Execute 'scrypto test' 

# TODO 

Implement a LazyMap containing ResourceAddress and Vaults instead of fixed crypto token pairs

## License

The Radix Scrypto Challenges code is released under Radix Modified MIT License.

    Copyright 2024 Radix Publishing Ltd

    Permission is hereby granted, free of charge, to any person obtaining a copy of
    this software and associated documentation files (the "Software"), to deal in
    the Software for non-production informational and educational purposes without
    restriction, including without limitation the rights to use, copy, modify,
    merge, publish, distribute, sublicense, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    This notice shall be included in all copies or substantial portions of the
    Software.

    THE SOFTWARE HAS BEEN CREATED AND IS PROVIDED FOR NON-PRODUCTION, INFORMATIONAL
    AND EDUCATIONAL PURPOSES ONLY.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
    FOR A PARTICULAR PURPOSE, ERROR-FREE PERFORMANCE AND NONINFRINGEMENT. IN NO
    EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES,
    COSTS OR OTHER LIABILITY OF ANY NATURE WHATSOEVER, WHETHER IN AN ACTION OF
    CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
    SOFTWARE OR THE USE, MISUSE OR OTHER DEALINGS IN THE SOFTWARE. THE AUTHORS SHALL
    OWE NO DUTY OF CARE OR FIDUCIARY DUTIES TO USERS OF THE SOFTWARE.