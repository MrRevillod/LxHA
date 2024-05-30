
import { ROLE } from "./lib/types"
import { Loading } from "./pages/Loading"
import { useAuth } from "./store/AuthContext"
import { useHttpStore } from "./store/HttpStore"
import { Navigate, Outlet } from "react-router-dom"

interface ProtectedRouteProps {
    protectedBy: string,
    redirectTo?: string
}

export const ProtectedRoute = ({ protectedBy, redirectTo = "/auth/login" }: ProtectedRouteProps) => {

    const { user, isAuthenticated, isCheckingSession, role, isLoading } = useAuth()

    if (isCheckingSession || isLoading) {
        return <Loading />
    }

    const conditions: { [key: string]: boolean } = {
        "role": !isAuthenticated || role !== ROLE.ADMINISTRATOR || !user,
        "session": !isAuthenticated || !user
    }

    if (conditions[protectedBy]) {
        return <Navigate to={redirectTo} replace />
    }

    return <Outlet />
}

export const LoadingWrapper = () => {

    const { isAuthenticated, isLoading } = useAuth()

    if (isLoading) {
        return <Loading />
    }

    if (isAuthenticated) {
        return <Navigate to="/dashboard" replace />
    }

    return <Outlet />
}