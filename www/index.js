import { calculate } from "color-counter";

const result = calculate({
    image_data: [0, 0, 0, 0],
    colors: {
        red: [255, 0, 0], //'#ff0000',
        white: [0, 0, 0],
    }
})

console.log('result', result)