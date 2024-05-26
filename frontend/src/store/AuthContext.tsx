
import Cookies from "js-cookie"

import { api } from "../lib/axios"
import { jwtDecode } from "jwt-decode"
import { useAppStore } from "./AppStore"
import { useHttpStore } from "./HttpStore"
import { createContext, PropsWithChildren, useContext } from "react"
import { Dispatch, SetStateAction, useEffect, useState } from "react"
import { JwtPayload, LoginData, RequestResetPasswordData, ROLE, User } from "../lib/types"

interface AuthStoreType {
    user: User | null,
    isAuthenticated: boolean,
    role: ROLE | null,
    isCheckingSession: boolean,
    setIsAuthenticated: Dispatch<SetStateAction<boolean>>,
    setRole: Dispatch<SetStateAction<ROLE | null>>,
    useLogin: (data: LoginData) => Promise<void>,
    setUser: Dispatch<SetStateAction<User | null>>,
    useLogout: () => Promise<void>
    useValidateSession: () => Promise<void>,
    useRequestResetPassword: (body: RequestResetPasswordData) => Promise<void>,
    useValidateResetPasswordPage: (id: string | undefined, token: string | undefined) => Promise<void>
    useResetPassword: (id: string | undefined, token: string | undefined, body: any) => Promise<void>
}

const defaultAuthStore: AuthStoreType = {
    user: null,
    isAuthenticated: false,
    role: null,
    isCheckingSession: false,
    setUser: () => { },
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

    const [user, setUser] = useState<User | null>(null)
    const [role, setRole] = useState<ROLE | null>(null)
    const [isAuthenticated, setIsAuthenticated] = useState(false)
    const [isCheckingSession, setIsCheckingSession] = useState(true)

    const { reset } = useAppStore()
    const { setResponse, setIsLoading } = useHttpStore()

    const useLogin = async (formData: LoginData) => {

        setIsLoading(true)

        try {

            const res = await api.post('/auth/login', formData)
            setUser(res.data.user)
            setIsAuthenticated(res.status === 200)
            setResponse(res.status, res.data.message, res.data, false)

            validateRoles()

        } catch (e: any) {

            console.log(e)

            setIsAuthenticated(false)
            setResponse(e.response.status, e.response.data.message, e.response.data, true)

        } finally { setIsLoading(false); reset() }
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

        } finally { setUser(null); setIsLoading(false); reset() }
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

    const validateRoles = () => {

        try {

            const sessionToken = Cookies.get("session") || ""
            const payload = jwtDecode<JwtPayload>(sessionToken)

            setRole(payload.role)

        } catch (error: any) {
            setRole(null)
            setIsAuthenticated(false)
            setResponse(error.response.status, error.response.data.message, error.response.data, false)
        }
    }

    const useValidateSession = async () => {

        setIsLoading(true)
        setIsCheckingSession(true)

        try {

            const res = await api.post("/auth/validate-session")

            validateRoles()

            setUser(res.data.user)
            setIsAuthenticated(res.status === 200)

        } catch (error: any) {

            console.log(error)

            setRole(null)
            setIsAuthenticated(false)
            setResponse(error.response.status, error.response.data.message, error.response.data, false)

        } finally { setIsCheckingSession(false); setIsLoading(false); reset() }
    }

    useEffect(() => {

        useValidateSession()

    }, [])

    const value: AuthStoreType = {
        user,
        isAuthenticated,
        role,
        isCheckingSession,
        setIsAuthenticated,
        setRole,
        setUser,
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
