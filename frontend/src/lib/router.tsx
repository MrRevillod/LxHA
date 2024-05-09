
import { useUserStore } from "../store/UserStore"
import { useAuthStore } from "../store/AuthStore"
import { Outlet, useNavigate } from "react-router-dom"
import { useEffect, PropsWithChildren, useState } from "react"

export const SessionProtectedRoute = ({ children }: PropsWithChildren) => {

    const navigate = useNavigate()

    const { user } = useUserStore()
    const { isAuthenticated, useValidateSession } = useAuthStore()
    
    const [isLoading, setIsloading] = useState<boolean>(true)

    useEffect(() => {

        (async function() {

            setIsloading(true)

            await useValidateSession()

            if (!user || !isAuthenticated) {
                return navigate("/auth/login")
            }

            setIsloading(false)

        })()

    }, [navigate, user, isAuthenticated])

    if (isLoading) return <h1 className="text-6xl text-red-950">LOADING ...</h1>

    return children ? children : <Outlet />
}

export const RoleProtectedRoute = ({ children }: PropsWithChildren) => {

    const navigate = useNavigate()

    const { user } = useUserStore()
    const { isAuthenticated, isAdmin, useValidatePermisions } = useAuthStore()

    const [isLoading, setIsloading] = useState<boolean>(true)

    useEffect(() => {

        (async function() {

            setIsloading(true)

            await useValidatePermisions()

            if (!user || !isAuthenticated || !isAdmin) {
                return navigate("/")
            }

            setIsloading(false)

        })()

    }, [navigate, user, isAuthenticated, isAdmin])

    if (isLoading) return <h1 className="text-6xl text-red-950">LOADING ...</h1>

    return children ? children : <Outlet />
}

