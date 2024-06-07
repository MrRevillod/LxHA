
import { Spinner } from "../components/ui/Spinner"

export const Loading = () => {

    return (

        <div className="z-50 h-screen w-screen flex items-center justify-center gap-4">
            <Spinner />
        </div>
    )
}
