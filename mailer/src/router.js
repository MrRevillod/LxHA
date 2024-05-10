
import { Router } from "express"
import { sender } from "./utils/mailer.js"
import { changeEmailTemplate, resetPasswordTemplate } from "./utils/templates.js"

export const router = Router()

router.post("/api/mailer/email-change", async (req, res) => {

    const { email, url } = await req.body

    const subject = "LXHA - Cambio de dirección de correo"

    sender(changeEmailTemplate, subject, email, url).then(() => {
        return res.status(200).json({ message: "Email sent" })

    }).catch(() => {
        return res.status(500).json({ message: "Internal server error" })
    })
})

router.post("/api/mailer/reset-password", async (req, res) => {

    const { email, url } = await req.body

    const subject = "LXHA - Restauración de contraseña"

    sender(resetPasswordTemplate, subject, email, url).then(() => {
        return res.status(200).json({ message: "Email sent" })

    }).catch(() => {
        return res.status(500).json({ message: "Internal server error" })
    })
})