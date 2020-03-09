import React from "react";

interface Props {}

const HomePageLayout = ({
  children,
  ...props
}: React.PropsWithChildren<Props>) => {
  return <div>{children}</div>;
};

export default HomePageLayout;
