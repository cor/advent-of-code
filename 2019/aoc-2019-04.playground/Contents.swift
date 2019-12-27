// MARK: - First Half
func increasesAndContainsDoubleDigit(_ number: Int) -> Bool {
    let digits = "\(number)".compactMap{ $0.wholeNumberValue }
    
    var foundDoubleDigit = false
    
    // Iterate over all digits
    for i in 0..<(digits.count - 1) {
        switch digits[i + 1] {
        case ..<digits[i]:
            return false // Digits are not increasing
        case digits[i]:
            foundDoubleDigit = true
        default: break
        }
    }
    
    return foundDoubleDigit
}


func passwordCount (range: ClosedRange<Int>) -> Int {
    return range
        .filter(increasesAndContainsDoubleDigit)
        .count
}


// MARK: - Example input
increasesAndContainsDoubleDigit(111111)
increasesAndContainsDoubleDigit(223450)
increasesAndContainsDoubleDigit(123789)


// MARK: - First half answer
print(passwordCount(range: 273025...767253))
