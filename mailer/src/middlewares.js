
import { AUTH_SERVICE_URL } from "./config.js"

export const authenticate = (protectedBy) => async (req, res, next) => {

    const authEndpoint = {
        ROLE: "validate-role",
        SESSION: "validate-session"
    }

    console.log(authEndpoint[protectedBy])

    try {

        const clientIp = req.headers['x-forwarded-for'] || req.connection.remoteAddress

        console.log(clientIp)

        const parsedCookies = `
            refresh=${req.cookies?.refresh || ""}; session=${req.cookies?.session || ""}
        `
        const authResponse = await fetch(`${AUTH_SERVICE_URL}/${authEndpoint[protectedBy]}`, {

            credentials: "include",
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "Cookie": parsedCookies,
                "x-forwarded-by": clientIp
            },
        })

        if (authResponse.status !== 200) {
            throw { status: authResponse.status, message: "Unauthorized" }
        }

        if (authResponse.headers.get("set-cookie") !== null) {
            res.cookie("session", authResponse.headers.get("set-cookie"))
        }

        next()

    } catch (error) {

        console.log(error)

        return res.status(error?.status || 500).json({ message: error?.message || "Internal server error" })
    }
}