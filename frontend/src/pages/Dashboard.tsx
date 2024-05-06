
import { useEffect } from 'react'
import { useAuthStore } from '../store/AuthStore'

export const DashboardPage = () => {

    const { useValidatePermisions, isLoading } = useAuthStore()

    useEffect(() => {
        const validate = async () => await useValidatePermisions()
        validate()
    }, [])

    return (

        <div className="h-40 bg-black flex flex-col items-center justify-center">
            
            <h1 className="text-white">Admin Dashboard (ROLE PROTECTED)</h1>
             
        </div>
    )
}
