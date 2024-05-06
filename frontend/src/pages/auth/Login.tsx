
import { useForm } from "react-hook-form"
import { useAuthStore } from "../../store/AuthStore"

export const LoginPage = () => {

    const { useLogin } = useAuthStore()
    const { register, handleSubmit } = useForm()

    const onSubmit = async (formdata: any) => await useLogin(formdata)

    return (

        <div className="bg-black flex flex-col items-center pt-8">
            
            <h1 className="text-white">LOGIN PAGE</h1>

            <form onSubmit={handleSubmit(onSubmit)} className="w-full flex flex-col gap-4 p-20">

              <input type="email" placeholder="email" className="h-8 rounded-lg" {...register("email")} />
              <input type="password" placeholder="password" className="h-8 rounded-lg" {...register("password")} />

              <button type="submit" className="bg-white h-8">Login</button>
            </form>
        </div>
    )
}
