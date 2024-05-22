
import { Spinner } from "../components/ui/Spinner"

export const Loading = () => {

    return (

        <div className="h-screen w-screen flex items-center justify-center gap-4 bg-neutral-100">
            <Spinner />
        </div>
    )
}