
import { Router } from "express"
import { sender } from "./utils/mailer.js"

import { MAILER_KEY } from "./config.js"
import { changeEmailTemplate, resetPasswordTemplate } from "./utils/templates.js"

export const router = Router()

const originValidation = (req, res, next) => {

    const apiKey = req.headers["x-api-key"]

    if (apiKey !== MAILER_KEY) {
        return res.status(401).json({ message: "Unauthorized" })
    }

    next()
}


router.post("/email-change", originValidation, async (req, res) => {

    const { email, url } = await req.body

    const subject = "LXHA - Cambio de dirección de correo"

    sender(changeEmailTemplate, subject, email, url).then(() => {
        return res.status(200).json({ message: "Email sent" })

    }).catch(() => {
        return res.status(500).json({ message: "Internal server error" })
    })
})

router.post("/forgot-password-email", originValidation, async (req, res) => {

    const { email, url } = await req.body

    const subject = "LXHA - Restauración de contraseña"
    
    sender(resetPasswordTemplate, subject, email, url).then(() => {
        return res.status(200).json({ message: "Email sent" })

    }).catch(() => {
        return res.status(500).json({ message: "Internal server error" })
    })
})