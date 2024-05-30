
export const toUpperAndLower = (str: string) => {
    return str.charAt(0).toUpperCase() + str.slice(1).toLowerCase()
}

export const toSuspensive = (str: string | number, th: number = 20) => {

    const threshold = th !== 20 ? th : 20

    if (typeof str === "number") str = str.toString()

    if (str.length > threshold) return str.slice(0, threshold - 5) + "..."

    return str
}
