(async () => {
    const {
        startService,
        build,
    } = require("esbuild")
    const service = await startService()

    try {
        const res = await service.build({
            entryPoints: ["./src/bind.ts"],
            outfile: './src/aplayer.min.js',
            loader: {
                '.svg': 'text'
            },
            format: 'esm',
            minify: true,
            bundle: true,
        })
    } finally {
        service.stop()
    }
})()
