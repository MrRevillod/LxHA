
import cors from "cors"
import express from "express"

import { router } from "./router.js"
import { MAILER_SERVICE_URL } from "./config.js"

const app = express()

app.use(cors({ origin: "*" }))

app.use(express.json())
app.use(express.urlencoded({ extended: true }))

app.use(router)

app.listen(5000, () => {
    console.log(`📧 Mailer service running on ${MAILER_SERVICE_URL}`)
})
