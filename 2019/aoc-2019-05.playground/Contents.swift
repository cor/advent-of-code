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
@discardableResult func executeIntcode(program inputProgram: IntcodeProgram) -> IntcodeProgram {
    
    // MARK: Machine state
    var program = inputProgram
    var instructionPointer = 0
    var running = true

    while (running && instructionPointer < program.count)
    {
        let opcode = program[instructionPointer]

        switch opcode.digits(2) {
        
        // MARK: Add
        case 1:
            let input1 = program[instructionPointer + 1]
            let input2 = program[instructionPointer + 2]
            let output = program[instructionPointer + 3]
            
            let operand1 = opcode.digit(2) == 1 ? input1 : program[input1]
            let operand2 = opcode.digit(3) == 1 ? input2 : program[input2]
            
            program[output] = operand1 + operand2
            
            instructionPointer += 4
            
        // MARK: Multiply
        case 2:
            let input1 = program[instructionPointer + 1]
            let input2 = program[instructionPointer + 2]
            let output = program[instructionPointer + 3]
            
            let operand1 = opcode.digit(2) == 1 ? input1 : program[input1]
            let operand2 = opcode.digit(3) == 1 ? input2 : program[input2]
            
            program[output] = operand1 * operand2
            
            instructionPointer += 4
            
        // MARK: Input
        case 3:
            let outputAddress = program[instructionPointer + 1]
    
            var userInput: Int? = nil
            
            for i in (1...5).reversed() {
                if (userInput == nil) {
                    if let inputString = readLine() {
                        if let inputInt = Int(inputString) {
                            userInput = inputInt
                        } else if i > 1 {
                            print("[INTCODE] Please input an Integer. \(i - 1) attempts left")
                        }
                    } else {
                        print("[INTCODE] ERROR: Could not get input from readLine(). Are you running from a playground?")
                    }
                }
            }
            
            if let input = userInput {
                program[outputAddress] = input
            } else {
                print("[INTCODE] ERROR: Did not receive valid user input")
                running = false
            }
            
            instructionPointer += 2
    
        // MARK: Output
        case 4:
            let input = program[instructionPointer + 1]
            
            print(program[input])
            
            instructionPointer += 2
            
        // MARK: Exit
        case 99:
            running = false
            
        // MARK: Invalid Opcode
        default:
            print("[INTCODE] ERROR: Invalid opcode")
            running = false
        }
    }
    
    return program
}

// MUST BE RUN FROM TERMINAL since readLine does not work in playgrounds
let challangeProgram = [3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1001, 191, 50, 224, 101, -64, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 5, 224, 224, 1, 224, 223, 223, 2, 150, 218, 224, 1001, 224, -1537, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 2, 224, 1, 223, 224, 223, 1002, 154, 5, 224, 101, -35, 224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 5, 224, 1, 224, 223, 223, 1102, 76, 17, 225, 1102, 21, 44, 224, 1001, 224, -924, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 4, 224, 1, 224, 223, 223, 101, 37, 161, 224, 101, -70, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 6, 224, 224, 1, 223, 224, 223, 102, 46, 157, 224, 1001, 224, -1978, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 5, 224, 1, 224, 223, 223, 1102, 5, 29, 225, 1101, 10, 7, 225, 1101, 43, 38, 225, 1102, 33, 46, 225, 1, 80, 188, 224, 1001, 224, -73, 224, 4, 224, 102, 8, 223, 223, 101, 4, 224, 224, 1, 224, 223, 223, 1101, 52, 56, 225, 1101, 14, 22, 225, 1101, 66, 49, 224, 1001, 224, -115, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 7, 224, 1, 224, 223, 223, 1101, 25, 53, 225, 4, 223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 108, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 329, 101, 1, 223, 223, 108, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 344, 1001, 223, 1, 223, 8, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 359, 101, 1, 223, 223, 7, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 374, 101, 1, 223, 223, 107, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 389, 101, 1, 223, 223, 7, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 404, 1001, 223, 1, 223, 1107, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 419, 1001, 223, 1, 223, 1007, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 434, 101, 1, 223, 223, 1008, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 449, 1001, 223, 1, 223, 1007, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 464, 1001, 223, 1, 223, 1008, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 479, 101, 1, 223, 223, 1007, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 494, 1001, 223, 1, 223, 108, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 509, 101, 1, 223, 223, 8, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 524, 1001, 223, 1, 223, 107, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 539, 101, 1, 223, 223, 107, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 554, 101, 1, 223, 223, 1107, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 569, 1001, 223, 1, 223, 1108, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 584, 1001, 223, 1, 223, 1008, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 599, 1001, 223, 1, 223, 1107, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 614, 101, 1, 223, 223, 7, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 629, 1001, 223, 1, 223, 1108, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 644, 1001, 223, 1, 223, 8, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 659, 101, 1, 223, 223, 1108, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 674, 101, 1, 223, 223, 4, 223, 99, 226]


// First Half Answer
executeIntcode(program: challangeProgram)


