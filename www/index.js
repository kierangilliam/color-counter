import { calculate } from "color-counter";

const canvas = document.getElementById('canvas')
const button = document.getElementById('button')
const result = document.getElementById('result')

const ctx = canvas.getContext("2d");
ctx.fillStyle = "red";
ctx.fillRect(0, 0, canvas.width / 2, canvas.height / 2);


button.addEventListener('click', () => {
    console.time('calc')

    const { data } = ctx.getImageData(0, 0, canvas.width, canvas.height)

    console.log(data)

    const { count } = calculate({
        data,//ctx.getImageData(0, 0, canvas.width, canvas.height),
        colors: {
            red: [255, 0, 0], //'#ff0000',
            white: [0, 0, 0],
        }
    })

    console.timeEnd('calc')

    result.innerHTML = JSON.stringify(count)
})