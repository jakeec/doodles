import React from "react";
import styled from "styled-components";

const Container = styled.div`
  display: flex;
  flex-direction: column;
  padding: 20px;
  overflow-y: auto;
`;

interface Props {}

const VerticalScrollLayout = ({
  children,
  ...props
}: React.PropsWithChildren<Props>) => {
  return <Container>{children}</Container>;
};

export default VerticalScrollLayout;
