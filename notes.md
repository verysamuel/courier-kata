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
      
# Part 2
I recognise many aspects of my implementation so far could be different.
- It would be totally valid to use a trait and unit structs (or even a map)
  to represent parcel types instead of an enum.
- I could instead implement the traits From<> and TryFrom<> where appropriate
    as alternatives for converting between types.
  
# Part 3
- Bit of an ambiguity in the requirements. +$2/kg could be discrete
or continuous. I have arbitrarily chosen to go for discrete, though
realistically I would seek to clarify this requirement.

# Part 4
- This part was quite simple really so not much to say.
Perhaps it was simple because of the structure I have gone with so far.
- Using an enum for the ParcelType has certainly been useful as the pattern
matching has saved me by requiring patterns be exhaustive.