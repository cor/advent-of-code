
// MARK: - First Half
struct WirePoint {
    var x: Int
    var y: Int
    
    static func += (left: inout WirePoint, right: WirePoint) {
        left.x += right.x
        left.y += right.y
    }
}

extension WirePoint: Hashable {
    var hash: Int {
        return x*y
    }
}

extension WirePoint: Equatable {
    static func == (lhs: WirePoint, rhs: WirePoint) -> Bool {
        return lhs.x == rhs.x && lhs.y == rhs.y
    }
}

typealias Wire = Set<WirePoint>
typealias WireDefinition = [Substring]

func generateWireDefinition(_ input: String) -> WireDefinition {
    return input.split(separator: ",")
}

func generateWire(definition: WireDefinition) -> Wire {
    
    let length = definition.map({Int($0.dropFirst())!}).reduce(0, +)
    
    var wire: Wire = Wire(minimumCapacity: length)
    var currentPosition = WirePoint(x: 0, y: 0)
    
    let positionDelta: [Character: WirePoint] = [
        "R": WirePoint(x: +1, y: 0),
        "L": WirePoint(x: -1, y: 0),
        "U": WirePoint(x: 0,  y: +1),
        "D": WirePoint(x: 0,  y: -1)
    ]
        
    for wireComponent in definition {
        let direction = wireComponent.first
        let length = Int(wireComponent.dropFirst())!
        
        for _ in 0..<abs(length) {
            currentPosition += positionDelta[direction!]!
            wire.insert(currentPosition)
        }
    }
    
    return wire
}

func findIntersections(wires: [Wire]) -> Set<WirePoint>
{
    return wires[0].intersection(wires[1])
}

func manhattanDistance(wirePoint: WirePoint) -> Int {
    return abs(wirePoint.x) + abs(wirePoint.y)
}


func part1Solution(_ input: [String]) -> Int {
    let wires = input
        .map(generateWireDefinition)
        .map(generateWire)

    return findIntersections(wires: wires)
        .map(manhattanDistance)
        .min()!
}


let exampleInput1 = ["R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"]
let exampleInput2 = ["R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"]

// First Half answer
let challangeInput = ["R995,U982,R941,U681,L40,D390,R223,U84,L549,U568,R693,D410,R779,U33,L54,D18,L201,U616,R583,D502,R307,U46,L726,D355,L62,D973,R134,U619,L952,U669,L28,U729,L622,D821,R814,D149,L713,U380,R720,U438,L112,U587,R161,U789,R959,U254,R51,U648,R259,U555,R863,U610,L33,D97,L825,D489,R836,D626,L849,D262,L380,U831,R650,U832,R339,D538,L49,D808,L873,D33,L405,D655,R884,D630,R589,D291,L675,D210,L211,D325,L934,D515,R896,U97,L639,U654,L301,U507,L642,D416,L325,U696,L3,U999,R88,D376,L187,U107,R394,U273,R117,D872,R162,D496,L599,D855,L961,U830,L156,U999,L896,D380,L476,U100,R848,U547,L266,D490,L87,D376,L428,U265,R645,U584,L623,D658,L103,U946,R162,U678,R532,D761,L141,D48,L487,D246,L85,D680,R859,D345,L499,D194,L815,D742,R700,D141,L482,D442,L943,D110,L892,D486,L581,U733,L109,D807,L474,U866,R537,U217,R237,U915,R523,D394,L509,U333,R734,U511,R482,D921,R658,U860,R112,U527,L175,D619,R140,D402,L254,D956,L556,U447,L518,U60,L306,U88,R311,U654,L551,D38,R750,U835,L784,U648,L374,U211,L635,U429,R129,U849,R411,D135,L980,U40,R480,D91,R881,D292,R474,D956,L89,D640,L997,D397,L145,D126,R936,U135,L167,U289,R560,D745,L699,U978,L459,D947,L782,U427,L784,D561,R985,D395,L358,D928,R697,U561,L432,U790,R112,D474,R852,U862,R721,D337,L355,U598,L94,D951,L903,D175,R981,D444,L690,D729,L537,D458,R883,U152,R125,D363,L90,U260,R410,D858,L825,U185,R502,D648,R793,D600,L589,U931,L772,D498,L871,U326,L587,D789,L934,D889,R621,U689,R454,U475,L166,U85,R749,D253,R234,D96,R367,D33,R831,D783,R577,U402,R482,D741,R737,D474,L666","L996,D167,R633,D49,L319,D985,L504,U273,L330,U904,R741,U886,L719,D73,L570,U982,R121,U878,R36,U1,R459,D368,R229,D219,R191,U731,R493,U529,R53,D613,R690,U856,L470,D722,R464,D378,L187,U540,R990,U942,R574,D991,R29,D973,R905,D63,R745,D444,L546,U939,L848,U860,R877,D181,L392,D798,R564,D189,R14,U120,R118,D350,R798,U462,R335,D497,R916,D722,R398,U91,L284,U660,R759,U676,L270,U69,L774,D850,R440,D669,L994,U187,R698,U864,R362,U523,L128,U89,R272,D40,L134,U571,L594,D737,L830,D956,L213,D97,R833,U454,R319,U809,L506,D792,R746,U283,R858,D743,R107,U499,R102,D71,R822,U9,L547,D915,L717,D783,L53,U871,R920,U284,R378,U312,R970,D316,R243,D512,R439,U530,R246,D824,R294,D726,R541,D250,R859,D134,R893,U631,L288,D151,L796,D759,R17,D656,L562,U136,R861,U42,L66,U552,R240,D121,L966,U288,L810,D104,R332,U667,L63,D463,R527,D27,R238,D401,L397,D888,R168,U808,L976,D462,R299,U385,L183,U303,L121,U385,R260,U80,R420,D532,R725,U500,L376,U852,R98,D597,L10,D441,L510,D592,L652,D230,L290,U41,R521,U726,R444,U440,L192,D255,R690,U141,R21,U942,L31,U894,L994,U472,L460,D357,R150,D721,R125,D929,R473,U707,R670,D118,R255,U287,R290,D374,R223,U489,R533,U49,L833,D805,L549,D291,R288,D664,R639,U866,R205,D746,L832,U864,L774,U610,R186,D56,R517,U294,L935,D304,L581,U899,R749,U741,R569,U282,R460,D925,L431,D168,R506,D949,L39,D472,R379,D125,R243,U335,L310,D762,R412,U426,L584,D981,L971,U575,L129,U885,L946,D221,L779,U902,R251,U75,L729,D956,L211,D130,R7,U664,L915,D928,L613,U740,R572,U733,R277,U7,R953,D962,L635,U641,L199"]

let answer = part1Solution(challangeInput)

print(answer)


// MARK: - PART TWO

func signalDelay(intersection: WirePoint, wireDefinition: WireDefinition) -> Int? {
    
    var delay = 0
    
    var currentPosition = WirePoint(x: 0, y: 0)
    
    let positionDelta: [Character: WirePoint] = [
        "R": WirePoint(x: +1, y: 0),
        "L": WirePoint(x: -1, y: 0),
        "U": WirePoint(x: 0,  y: +1),
        "D": WirePoint(x: 0,  y: -1)
    ]
        
    for wireComponent in wireDefinition {
        let direction = wireComponent.first
        let length = Int(wireComponent.dropFirst())!
        
        for _ in 0..<abs(length) {
            currentPosition += positionDelta[direction!]!
            delay += 1
            
            if currentPosition == intersection {
                return delay
            }
        }
    }
    
    return nil
}

func signalDelays(intersection: WirePoint, wireDefinitions: [WireDefinition]) -> Int? {
    return wireDefinitions
        .map{signalDelay(intersection: intersection, wireDefinition: $0)!}
        .reduce(0, +)
}

func part2Solution(_ input: [String]) -> Int {
    let wireDefinitions = input.map(generateWireDefinition)
    let wires = wireDefinitions.map(generateWire)
    let intersections = findIntersections(wires: wires)
    
    
    return intersections
        .map{signalDelays(intersection: $0, wireDefinitions: wireDefinitions)!}
        .min()!
}

// Example input
part2Solution(["R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"])
part2Solution(["R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"])

let answer2 = part2Solution(challangeInput)
print(answer2)
