import APlayer from "aplayer";


type Audio = {
    name: String,
    url: String,
    artist: String,
    cover: String,
    lrc: String,
    theme: String,
    type: 'auto' | 'hls' | 'normal'
}

type AplayerOptions = {
    fixed: Boolean,
    mini: Boolean,
    autoplay: Boolean,
    theme: String,
    loop: 'all' | 'one' | 'none',
    order: 'list' | 'random',
    preload: 'none' | 'metadata' | 'auto',
    volume: number,
    mutex: Boolean,
    listFolded: Boolean,
    listMaxHeight: number,
    lrcType: 0 | 1 | 2 | 3,
    audio: Audio[],
    storageName: String
}


export function build_aplayer(e: HTMLElement, o: AplayerOptions) {
    new APlayer({ container: e, ...o });
}

