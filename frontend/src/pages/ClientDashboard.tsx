
import { useEffect } from 'react'
import { useAuthStore } from '../store/AuthStore'

export const ClientDashboardPage = () => {

    const { useValidateSession, useLogout } = useAuthStore()

    useEffect(() => {
        const validate = async () => await useValidateSession()
        validate()
    }, [])

    const handleLogout = async () => await useLogout()

    return (

        <div className="h-40 bg-black flex flex-col items-center justify-center">
            
            <h1 className="text-white">Client Dashboard (SESSION PROTECTED)</h1>

        </div>
    )
}
