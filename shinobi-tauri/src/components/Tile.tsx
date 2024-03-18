import './Tile.css';

interface Props {
    dark: boolean
    image?: string
}


export default function Tile({ dark, image: image }: Props) {
    if (dark) {
        return (
            <div class='tile dark-tile'>
                {image && <div class="piece-image" style={{ "background-image": `url(${image})` }}></div>}
            </div>
        );
    }
    else {
        return (
            <div class='tile light-tile'>
                {image && <div class="piece-image" style={{ "background-image": `url(${image})` }}></div>}
            </div>
        );
    }
}
