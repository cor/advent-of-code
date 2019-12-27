import Foundation

//  _____ _   _ _______ _____ ____  _____  ______
// |_   _| \ | |__   __/ ____/ __ \|  __ \|  ____|
//   | | |  \| |  | | | |   | |  | | |  | | |__
//   | | | . ` |  | | | |   | |  | | |  | |  __|
//  _| |_| |\  |  | | | |___| |__| | |__| | |____
// |_____|_| \_|  |_|  \_____\____/|_____/|______|
//

// OPCODES:

// 1 - ADD:
// arguments: inputAddress1 inputAddress2 outputAddress
// store sum of inputAddress1 and inputAddress2 at outputAddress

// 2 - MULTIPLY:
// arguments: inputAddress1 inputAddress2 outputAddress
// store product of inputAddress1 and inputAddress2 at outputAddress

// 99 - EXIT:
// immediately end program

// increase program counter by 4 after executing instruction

// MARK: - Int Extensions
typealias IntcodeProgram = [Int]

precedencegroup PowerPrecedence { higherThan: MultiplicationPrecedence }
infix operator ^^ : PowerPrecedence
func ^^ (radix: Int, power: Int) -> Int {
    return Int(pow(Double(radix), Double(power)))
}

extension Int {
    func digit(_ position: Int) -> Int {
        return self / (10 ^^ position) % 10
    }
    
    func digits(_ amount: Int) -> Int {
        return self % (10 ^^ amount)
    }
}

// MARK: - Intcode computer
func executeIntcode(program inputProgram: IntcodeProgram) -> IntcodeProgram {
    var program = inputProgram
    var instructionPointer = 0
    var running = true

    while (running && instructionPointer < program.count)
    {
        let opcode = program[instructionPointer]

        switch opcode.digits(2) {
        case 1: // Add
            let inputAddress1 = program[instructionPointer + 1]
            let inputAddress2 = program[instructionPointer + 2]
            let outputAddress = program[instructionPointer + 3]
            
            program[outputAddress] = program[inputAddress1] + program[inputAddress2]
            
            instructionPointer += 4
            
        case 2: // Multiply
            let inputAddress1 = program[instructionPointer + 1]
            let inputAddress2 = program[instructionPointer + 2]
            let outputAddress = program[instructionPointer + 3]
            
            program[outputAddress] = program[inputAddress1] * program[inputAddress2]
            
            instructionPointer += 4
            
        case 4: // Output
            let outputAddres = program[instructionPointer + 1]
            
            print(program[outputAddres])
            
            instructionPointer += 2
            
        case 99: // Exit
            running = false
            
        default:
            print("ERROR: Invalid opcode")
            running = false
        }
        
        // Go to next instruction
    }
    
    return program
}


executeIntcode(program: [23030301, 0, 0, 1, 4, 1])

