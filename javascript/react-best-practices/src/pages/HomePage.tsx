import React from "react";
import HomePageLayout from "../layouts/HomePageLayout";
import Newsfeed from "../containers/Newsfeed";

interface Props {
  theme?: string;
}

const HomePage = ({ children, ...props }: React.PropsWithChildren<Props>) => {
  return (
    <HomePageLayout>
      <Newsfeed />
    </HomePageLayout>
  );
};

export default HomePage;
