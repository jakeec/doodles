import React from "react";
import VerticalScrollLayout from "../layouts/VerticalScrollLayout";
import NewsfeedItem from "./NewfeedItem";

interface Props {}

const Newsfeed = ({ children, ...props }: React.PropsWithChildren<Props>) => {
  return (
    <VerticalScrollLayout>
      <NewsfeedItem title="Check this out" body="This is test text" />
    </VerticalScrollLayout>
  );
};

export default Newsfeed;
