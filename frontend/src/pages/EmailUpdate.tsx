
import { Helmet } from "react-helmet"
import { Loading } from "./Loading"
import { useParams } from "react-router-dom"
import { useHttpStore } from "../store/HttpStore"
import { useEffect, useState } from "react"
import { useUserStore } from "../store/UserStore"
import { MainLayout } from "../layouts/MainLayout"

export const EmailUpdatePage = () => {

    const { id, token } = useParams()
    const { message, isLoading } = useHttpStore()
    const [isValidating, setIsValidating] = useState(true)
    const { emailUpdate } = useUserStore()

    useEffect(() => {

        const validatePage = async () => {
            setIsValidating(true)
            await emailUpdate(id || "", token || "")
            setIsValidating(false)
        }

        validatePage()

    }, [])

    if (isLoading || isValidating) {
        return <Loading />
    }

    return (

        <MainLayout>

            <Helmet>
                <title>Lx High Availability - Password Reset</title>
            </Helmet>

            <div className="w-full h-full flex justify-center items-center">
                <h1 className="text-5xl text-center font-bold">{message}</h1>
            </div>

        </MainLayout>
    )
}
