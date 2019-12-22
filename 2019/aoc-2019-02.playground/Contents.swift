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

typealias IntcodeProgram = [Int]

func executeIntcode(program inputProgram: IntcodeProgram) -> IntcodeProgram {
    var program = inputProgram
    var programCounter = 0
    var running = true

    while (running && programCounter < program.count)
    {
        let opcode = program[programCounter]

        switch opcode {
        case 1: // Add
            let inputAddress1 = program[programCounter + 1]
            let inputAddress2 = program[programCounter + 2]
            let outputAddress = program[programCounter + 3]
            program[outputAddress] = program[inputAddress1] + program[inputAddress2]
        case 2: // Multiply
            let inputAddress1 = program[programCounter + 1]
            let inputAddress2 = program[programCounter + 2]
            let outputAddress = program[programCounter + 3]
            program[outputAddress] = program[inputAddress1] * program[inputAddress2]
        case 99: // Exit
            running = false
        default:
            print("ERROR: Invalid opcode")
            running = false
        }
        
        programCounter += 4 // Go to next instruction
    }
    
    return program
}

// Example Inputs
executeIntcode(program: [1, 0, 0, 0, 99])
executeIntcode(program: [2, 3, 0, 3, 99])
executeIntcode(program: [2, 4, 4, 5, 99, 0])
executeIntcode(program: [1, 1, 1, 4, 99, 5, 6, 0, 99])

let challangeInput = [1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,1,9,19,23,2,13,23,27,2,27,13,31,2,31,10,35,1,6,35,39,1,5,39,43,1,10,43,47,1,5,47,51,1,13,51,55,2,55,9,59,1,6,59,63,1,13,63,67,1,6,67,71,1,71,10,75,2,13,75,79,1,5,79,83,2,83,6,87,1,6,87,91,1,91,13,95,1,95,13,99,2,99,13,103,1,103,5,107,2,107,10,111,1,5,111,115,1,2,115,119,1,119,6,0,99,2,0,14,0]

// First Half Answer
executeIntcode(program: challangeInput)
