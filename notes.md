# Notes on Courier Kata

## Part 1
Things that may be important to consider before I start:
- Dimensions
    - Dimensions can't be equal to or less than 0.
      This can be encoded into a newtype.
- Cost
    - Use a representation that doesn't lead to rounding errors (floats)
      and is suitably flexible.
      Decimal type is tempting but for this but
      representing cents as an integer might be simpler
      and sufficiently effective.
    - Tempting to make this a newtype as well, but I will reserve that
      for if there are any invariants to manage for Costs.
- Parcel types
    - Making expansion of requirements (new types of parcels) simple enough
      without too much up-front effort.
