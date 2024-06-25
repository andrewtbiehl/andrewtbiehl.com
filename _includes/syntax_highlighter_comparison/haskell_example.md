```haskell
import Control.Monad (mfilter)
import Data.Maybe (isJust)
import Text.Read (readMaybe)

digitSum :: Integer -> Integer
digitSum 0 = 0
digitSum n = (n `mod` 10) + digitSum (n `div` 10)

checkDigit :: [Integer] -> Integer
checkDigit = (10 -) . (`mod` 10) . checksum
  where
    checksum = sum . map digitSum . doubleEveryOther
    doubleEveryOther = zipWith ($) (cycle [id, (* 2)])

isValidLuhnSequence :: [Integer] -> Bool
isValidLuhnSequence = (==) <$> calculatedCheckDigit <*> givenCheckDigit
  where
    givenCheckDigit = last
    calculatedCheckDigit = checkDigit . init

main = do
  putStrLn "Input a number to validate:"
  input <- getLine
  let response = if isValidLuhnNumber input then "Valid!" else "Not valid."
  putStrLn response
  where
    isValidLuhnNumber = isJust . (mfilter isValidLuhnSequence) . digits
    digits = mapM readMaybe . map (\c -> [c])
```
{: highlighter="compare"}
