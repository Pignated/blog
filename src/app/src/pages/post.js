import { useParams } from "react-router-dom";


function Post() {
    const{postID} = useParams();
    return (
        <> {postID}
        </>
    )

}
export default Post;