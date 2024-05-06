
import { useParams } from 'react-router-dom'
import { useAuthStore } from '../../store/AuthStore'
import { useEffect } from 'react'

export const ValidateAccountPage = () => {

    const { id, token } = useParams()
    const { useValidateAccount } = useAuthStore()

    useEffect(() => {

        (async () => await useValidateAccount(id, token))()

    }, [])

    return (

        <div className="h-40 bg-black flex flex-col items-center justify-center">
            
            <h1 className="text-white">VALIDATE ACCOUNT PAGE</h1>
            
        </div>
    )
}
