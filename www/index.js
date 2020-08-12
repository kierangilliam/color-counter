import { calculate } from "color-counter";

const canvas = document.getElementById('canvas')
const button = document.getElementById('button')
const result = document.getElementById('result')

const ctx = canvas.getContext("2d");
ctx.fillStyle = "red";
ctx.fillRect(0, 0, canvas.width / 2, canvas.height / 2);


button.addEventListener('click', () => {
    console.time('calc')

    const image = ctx.getImageData(0, 0, canvas.width, canvas.height)

    console.time('array from')
    const data = Array.from(image.data)
    console.timeEnd('array from')

    const { count } = calculate({
        data,
        colors: {
            red: [255, 0, 0],
            white: [0, 0, 0],
        }
    })

    console.timeEnd('calc')

    result.innerHTML = JSON.stringify(count)
})