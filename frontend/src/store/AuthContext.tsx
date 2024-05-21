
import Cookies from "js-cookie"

import { api } from "../lib/axios"
import { jwtDecode } from "jwt-decode"
import { useUserStore } from "./UserStore"
import { useHttpStore } from "./HttpStore"
import { createContext, PropsWithChildren, useContext } from "react"
import { Dispatch, SetStateAction, useEffect, useState } from "react"
import { JwtPayload, LoginData, RequestResetPasswordData, ROLE } from "../lib/types"

interface AuthStoreType {
    isAuthenticated: boolean,
    role: ROLE | null,
    isCheckingSession: boolean,
    setIsAuthenticated: Dispatch<SetStateAction<boolean>>,
    setRole: Dispatch<SetStateAction<ROLE | null>>,
    useLogin: (data: LoginData) => Promise<void>,
    useLogout: () => Promise<void>
    useValidateSession: () => Promise<void>,
    useRequestResetPassword: (body: RequestResetPasswordData) => Promise<void>,
    useValidateResetPasswordPage: (id: string | undefined, token: string | undefined) => Promise<void>
    useResetPassword: (id: string | undefined, token: string | undefined, body: any) => Promise<void>
}

const defaultAuthStore: AuthStoreType = {
    isAuthenticated: false,
    role: null,
    isCheckingSession: false,
    setIsAuthenticated: () => { },
    setRole: () => { },
    useLogin: async () => { },
    useLogout: async () => { },
    useValidateSession: async () => { },
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

    const [role, setRole] = useState<ROLE | null>(null)
    const [isAuthenticated, setIsAuthenticated] = useState(false)
    const [isCheckingSession, setIsCheckingSession] = useState(true)

    const { setUser } = useUserStore()
    const { setResponse, setIsLoading } = useHttpStore()

    const useLogin = async (formData: LoginData) => {

        setIsLoading(true)

        try {

            const res = await api.post('/auth/login', formData)
            setUser(res.data.user)
            setIsAuthenticated(res.status === 200)
            setResponse(res.status, res.data.message, res.data, false)

            await useValidateSession()

        } catch (e: any) {
            setIsAuthenticated(false)
            setResponse(e.response.status, e.response.data.message, e.response.data, true)

        } finally { setIsLoading(false) }
    }

    const useLogout = async () => {

        setIsLoading(true)

        try {

            const res = await api.post('/auth/logout')
            setIsAuthenticated(!(res.status === 200))
            setResponse(res.status, res.data.message, res.data, true)

        } catch (e: any) {
            setRole(null)
            setIsAuthenticated(false)
            setResponse(e.response.status, e.response.data.message, e.response.data, true)

        } finally { setUser(null); setIsLoading(false) }
    }

    const useRequestResetPassword = async (formData: RequestResetPasswordData) => {

        setIsLoading(true)

        try {

            const res = await api.post('/auth/reset-password', formData)
            setResponse(res.status, res.data.message, res.data, true)

        } catch (e: any) {
            setResponse(e.response.status, e.response.data.message, e.response.data, true)

        } finally { setIsLoading(false) }
    }

    const useResetPassword = async (id: string | undefined, token: string | undefined, formData: any) => {

        setIsLoading(true)

        try {

            const res = await api.patch(`/auth/reset-password/${id}/${token}`, formData)
            setResponse(res.status, res.data.message, res.data, true)

        } catch (e: any) {
            setResponse(e.response.status, e.response.data.message, e.response.data, true)

        } finally { setIsLoading(false) }

    }

    const useValidateResetPasswordPage = async (id: string | undefined, token: string | undefined) => {

        setIsLoading(true)

        try {

            const res = await api.post(`/auth/reset-password/${id}/${token}`)
            setResponse(res.status, res.data.message, res.data, false)

        } catch (error: any) {
            setResponse(error.response.status, error.response.data.message, error.response.data, false)

        } finally { setIsLoading(false) }
    }

    const useValidateSession = async () => {

        setIsLoading(true)
        setIsCheckingSession(true)

        try {

            const sessionToken = Cookies.get("session")

            if (!sessionToken) return setIsAuthenticated(false)

            const res = await api.post("/auth/validate-session")
            const payload = jwtDecode<JwtPayload>(sessionToken)

            setRole(payload.role)
            setUser(res.data.user)
            setIsAuthenticated(res.status === 200)

        } catch (e: any) {
            setRole(null)
            setIsAuthenticated(false)
            setResponse(e.response.status, e.response.data.message, e.response.data, true)

        } finally { setIsCheckingSession(false); setIsLoading(false) }
    }

    useEffect(() => { useValidateSession() }, [])

    const value: AuthStoreType = {
        isAuthenticated,
        role,
        isCheckingSession,
        setIsAuthenticated,
        setRole,
        useLogin,
        useLogout,
        useRequestResetPassword,
        useValidateResetPasswordPage,
        useResetPassword,
        useValidateSession,
    }

    return (

        <AuthContext.Provider value={value}>
            {children}
        </AuthContext.Provider>
    )
}
