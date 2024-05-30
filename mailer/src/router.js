
import { Router } from "express"
import { authenticate } from "./middlewares.js"

import {
    adminToUserMessageController, newAccountMessageController,
    newInstanceMessageController, resetPasswordController,
    updateEmailController, userToAdminMessageController
} from "./controllers.js"

export const router = Router()

router.post("/api/mailer/contact-test", authenticate("ROLE"), (req, res) => {
    res.status(200).send("Contact test successful")
})

router.post("/api/mailer/email-change", updateEmailController)
router.post("/api/mailer/reset-password", resetPasswordController)

router.post("/api/mailer/support-req", authenticate("SESSION"), userToAdminMessageController)
router.post("/api/mailer/admin-message", authenticate("ROLE"), adminToUserMessageController)

router.post("/api/mailer/new-instance", newInstanceMessageController)
router.post("/api/mailer/new-account", newAccountMessageController)