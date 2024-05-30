
import { sender } from "./utils/mailer.js"
import { changeEmailTemplate, resetPasswordTemplate } from "./utils/templates.js"

export const updateEmailController = async (req, res) => {

    const { email, url } = await req.body

    const subject = "LXHA - Cambio de dirección de correo"

    sender(changeEmailTemplate, subject, email, url).then(() => {
        return res.status(200).json({ message: "Email sent" })

    }).catch(() => {
        return res.status(500).json({ message: "Internal server error" })
    })
}

export const resetPasswordController = async (req, res) => {

    const { email, url } = await req.body

    const subject = "LXHA - Restauración de contraseña"

    sender(resetPasswordTemplate, subject, email, url).then(() => {
        return res.status(200).json({ message: "Email sent" })

    }).catch(() => {
        return res.status(500).json({ message: "Internal server error" })
    })
}

export const userToAdminMessageController = async (req, res) => {

    // Directly from frontend (message from client to administrator)

    return res.status(200).json({ message: "OK" })
}

export const adminToUserMessageController = async (req, res) => {

    // Directly from frontend (message from admin to user)

    return res.status(200).json({ message: "OK" })
}

export const newInstanceMessageController = async (req, res) => {
    return res.status(200).json({ message: "OK" })
}

export const newAccountMessageController = async (req, res) => {
    return res.status(200).json({ message: "OK" })
}