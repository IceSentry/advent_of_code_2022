const text = (await Deno.readTextFile("inputs/2022/02.txt")).trim()

const numbers: {[value:string]:number} = {
    A: 1, //rock
    B: 2, //paper
    C: 3, //scissors
    X: 1, //rock
    Y: 2, //paper
    Z: 3  //scissors
}

async function part1() {
    let score = 0
    text.split("\n").map(line => {
        const p1 = numbers[line[0]]
        const p2 = numbers[line[2]]
        score += p2
        if (p1 == p2)
            score += 3
        else if (p2%3 == p1%3+1)
        // else if (p2 > p1)
            score += 6
    })
    console.log(score)
}

console.log("hello")

await part1()