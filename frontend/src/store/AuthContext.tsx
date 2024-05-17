
import { api } from "../lib/axios"
import { useUserStore } from "./UserStore"
import { createContext, PropsWithChildren, useContext } from "react"
import { Dispatch, SetStateAction, useEffect, useState } from "react"
import { LoginData, RequestResetPasswordData } from "../lib/types"
import { AxiosResponse } from "axios"
import { useHttpStore } from "./HttpStore"

interface AuthStoreType {
    isAuthenticated: boolean,
    isAdmin: boolean,
    isCheckingSession: boolean,
    setIsAuthenticated: Dispatch<SetStateAction<boolean>>,
    setIsAdmin: Dispatch<SetStateAction<boolean>>,
    useLogin: (data: LoginData) => Promise<void>,
    useLogout: () => Promise<void>
    useValidatePermissions: () => Promise<void>,
    useRequestResetPassword: (body: RequestResetPasswordData) => Promise<void>,
    useValidateResetPasswordPage: (id: string | undefined, token: string | undefined) => Promise<void> | Promise<AxiosResponse<any, any>>,
    useResetPassword: (id: string | undefined, token: string | undefined, body: any) => Promise<void>
}

const defaultAuthStore: AuthStoreType = {
    isAuthenticated: false,
    isAdmin: false,
    isCheckingSession: false,
    setIsAuthenticated: () => { },
    setIsAdmin: () => { },
    useLogin: async () => { },
    useLogout: async () => { },
    useValidatePermissions: async () => { },
    useRequestResetPassword: async () => { },
    useValidateResetPasswordPage: async () => { },
    useResetPassword: async () => { }
}

export const AuthContext = createContext<AuthStoreType>(defaultAuthStore)

export const useAuth = () => {

    const context = useContext(AuthContext)

    if (!context) throw new Error("useAuth debe estar dentro del proveedor AuthContext")
    return context
}

export const AuthProvider = ({ children }: PropsWithChildren) => {

    const [isAdmin, setIsAdmin] = useState(false)
    const [isAuthenticated, setIsAuthenticated] = useState(false)
    const [isCheckingSession, setIsCheckingSession] = useState(true)

    const { setUser } = useUserStore()
    const { setResponse, setIsLoading } = useHttpStore()

    const useLogin = async (formData: LoginData) => {

        try {

            setIsLoading(true)

            const res = await api.post('/auth/login', formData)

            setUser(res.data.user)
            setIsAuthenticated(res.status === 200)
            setResponse(res.status, res.data.message, res.data, false)

            await useValidatePermissions()

        } catch (e: any) {

            setIsAuthenticated(false)
            setResponse(e.response.status, e.response.data.message, e.response.data, true)

        } finally { setIsLoading(false) }
    }

    const useLogout = async () => {

        try {

            setIsLoading(true)

            const res = await api.post('/auth/logout')

            setIsAuthenticated(!(res.status === 200))
            setResponse(res.status, res.data.message, res.data, true)

        } catch (e: any) {

            setIsAdmin(false)
            setIsAuthenticated(false)
            setResponse(e.response.status, e.response.data.message, e.response.data, true)

        } finally { setUser(null); setIsLoading(false) }
    }

    const useRequestResetPassword = async (formData: RequestResetPasswordData) => {

        try {

            setIsLoading(true)

            const res = await api.post('/auth/reset-password', formData)

            setResponse(res.status, res.data.message, res.data, true)

        } catch (e: any) {
            setResponse(e.response.status, e.response.data.message, e.response.data, true)

        } finally { setIsLoading(false) }
    }

    const useResetPassword = async (id: string | undefined, token: string | undefined, formData: any) => {

        try {

            setIsLoading(true)

            const res = await api.patch(`/auth/reset-password/${id}/${token}`, formData)

            setResponse(res.status, res.data.message, res.data, true)

        } catch (e: any) {
            setResponse(e.response.status, e.response.data.message, e.response.data, true)

        } finally { setIsLoading(false) }

    }

    const useValidateResetPasswordPage = async (id: string | undefined, token: string | undefined) => {

        try {

            setIsLoading(true)
            return await api.post(`/auth/reset-password/${id}/${token}`)

        } catch (error: any) {
            return error.response

        } finally { setIsLoading(false) }

    }

    const checkSession = async () => {

        try {

            setIsLoading(true)
            setIsCheckingSession(true)

            const res = await api.post('/auth/validate-session')

            setUser(res.data.user)
            setIsAuthenticated(res.status === 200)

        } catch (error) {

            setIsAuthenticated(false)
            setIsAdmin(false)
            setUser(null)

        } finally {
            setIsLoading(false)
            setIsCheckingSession(false)
        }
    }

    const useValidatePermissions = async () => {

        try {

            setIsLoading(true)
            setIsCheckingSession(true)

            const res = await api.post('/auth/validate-role')

            setIsAdmin(res.status === 200)
            setUser(res.data.user)

        } catch (e: any) {
            setIsAdmin(false)

        } finally {
            setIsLoading(false)
            setIsCheckingSession(false)
        }
    }

    useEffect(() => { checkSession(); useValidatePermissions() }, [])

    const value: AuthStoreType = {
        isAuthenticated,
        isAdmin,
        isCheckingSession,
        setIsAuthenticated,
        setIsAdmin,
        useLogin,
        useLogout,
        useValidatePermissions,
        useRequestResetPassword,
        useValidateResetPasswordPage,
        useResetPassword
    }

    return (

        <AuthContext.Provider value={value}>
            {children}
        </AuthContext.Provider>
    )
}
