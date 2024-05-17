
import { useAuth } from "../store/AuthContext"
import { Link, useLocation } from "react-router-dom"

import "../index.css"

export const Navbar = () => {

    const location = useLocation()
    const isDashboardOrHome = location.pathname === "/" || location.pathname === "/dashboard"

    const { isAuthenticated, useLogout } = useAuth()

    return (

        <div className="navbar bg-neutral-950 py-8 px-6 md:px-12 lg:px-28 fixed top-0 w-full z-50 h-28">
            <div className="flex-1">

                {isDashboardOrHome && (

                    <div className="flex flex-row gap-4">
                        <Link
                            to="/"
                            className="
                            text-neutral-300 font-bold text-2xl hover:text-neutral-400 transition duration-200
                        "
                        >
                            Lxd High Av

                        </Link>

                        <Link
                            to="/auth/login"
                            className="
                            text-neutral-300 font-bold text-2xl hover:text-neutral-400 transition duration-200
                        ">
                            Go to login
                        </Link>
                    </div>
                )}

                {!isDashboardOrHome && (

                    <Link
                        to="/dashboard"
                        className="
                            text-neutral-300 flex items-center justify-center font-semibold 
                            text-xl hover:text-neutral-400 transition duration-200 back
                        "
                    >

                        volver

                    </Link>
                )}

            </div>
            <div className="flex-none">

                {isAuthenticated && (
                    <>
                        <button onClick={useLogout}
                            className="bg-neutral-100 rounded-lg px-8 py-2 text-neutral-950 font-bold text-md">
                            Cerrar sesión
                        </button>
                    </>
                )}

                {!isAuthenticated && (

                    <Link
                        to="/auth/login"
                        className="bg-neutral-100 rounded-lg px-8 py-2 text-neutral-950 font-bold text-md"
                    >
                        Iniciar sesión

                    </Link>
                )}

            </div>
        </div >
    )
}

