import React from "react";
import styled from "styled-components";

const getNthChild = (children, i) => {
  if (Array.isArray(children)) {
    return React.cloneElement(children[i]);
  } else if (typeof children === "object") {
    console.log(children);
    return React.cloneElement(children);
  } else {
    return children;
  }
};

const Container = styled.div`
  display: flex;
  flex-direction: column;
  min-height: 150px;
  padding: 10px;
  border-radius: 10px;
  background: white;
`;

interface Props {}

const CardLayout = ({ children, ...props }: React.PropsWithChildren<Props>) => {
  return (
    <Container>
      {getNthChild(children, 0)}
      {getNthChild(children, 1)}
      Test
    </Container>
  );
};

export default CardLayout;
