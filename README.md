### Usage
```
<canvas id="canvas" width="3000" height="1500"></canvas>
```

```
import { calculate } from 'color-counter'

const canvas = document.getElementById('canvas')
const ctx = canvas.getContext('2d')

// Fill 1/4th of canvas with red
ctx.fillStyle = 'red'
ctx.fillRect(0, 0, canvas.width / 2, canvas.height / 2)

const { data } = ctx.getImageData(0, 0, canvas.width, canvas.height)

const { count } = calculate({
    data: Array.from(data),
    colors: {
        red: [255, 0, 0],
        white: [0, 0, 0],
    }
})

// {"white": 3375000, "red": 1125000, "unknown": 0}
console.log(count)
```

### Disclaimer

First time ever using rust or wasm so this is definitely the worst way to do this.

### Build

```
wasm-pack build
```

### Test

```
wasm-pack test --headless --firefox
```

### Publish

```
wasm-pack publish
```

