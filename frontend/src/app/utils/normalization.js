export function shortNameToLogoPath(shortName) {
    let imgname = shortName
        .toLowerCase()
        .normalize("NFKD")
        .trim()
        .replace(/[\u0300-\u036f]/g, "")
        .replace(/\s/, "-")
    return `/img/${imgname}.png`
}