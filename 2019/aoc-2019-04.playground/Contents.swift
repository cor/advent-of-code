// MARK: - First Half
func increasesAndContainsDoubleDigit(_ number: Int, noMoreThanTwoDigits: Bool) -> Bool {
    let digits = "\(number)".compactMap{ $0.wholeNumberValue }
    
    var foundDoubleDigit = false
    
    // Iterate over all digits
    for i in 0..<(digits.count - 1) {
        if (digits[i + 1] < digits[i]) { // Check if digits are ascending
            return false
        }
        
        if (digits[i + 1] == digits[i]) { // Check for double digits
            if noMoreThanTwoDigits {
                let precedantIsDifferent = i > 0 ? digits[i - 1] != digits[i] : true
                let succesorIsDifferent = i + 2 < digits.count ? digits[i + 2] != digits[i] : true
                
                if precedantIsDifferent && succesorIsDifferent {
                    foundDoubleDigit = true
                }
            } else {
                foundDoubleDigit = true
            }
        }
    }
    
    return foundDoubleDigit
}


func passwordCount (range: ClosedRange<Int>, noMoreThanTwoDigits: Bool) -> Int {
    return range
        .filter{increasesAndContainsDoubleDigit($0, noMoreThanTwoDigits: noMoreThanTwoDigits)}
        .count
}


// MARK: Example input
increasesAndContainsDoubleDigit(111111, noMoreThanTwoDigits: false)
increasesAndContainsDoubleDigit(223450, noMoreThanTwoDigits: false)
increasesAndContainsDoubleDigit(123789, noMoreThanTwoDigits: false)


// MARK: First half answer
print(passwordCount(range: 273025...767253, noMoreThanTwoDigits: false))


// MARK: - Second Half

// MARK: Example Input
increasesAndContainsDoubleDigit(112233, noMoreThanTwoDigits: true)
increasesAndContainsDoubleDigit(123444, noMoreThanTwoDigits: true)
increasesAndContainsDoubleDigit(111122, noMoreThanTwoDigits: true)

// MARK: Second half answer
print(passwordCount(range: 273025...767253, noMoreThanTwoDigits: true))
