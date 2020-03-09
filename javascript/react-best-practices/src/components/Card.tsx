import React from "react";
import styled from "styled-components";
import CardLayout from "../layouts/CardLayout";

const Container = styled.div`
  display: flex;
  min-height: 150px;
  padding: 10px;
  border-radius: 10px;
  background: white;
`;

interface Props {}

const Card = ({ children, ...props }: React.PropsWithChildren<Props>) => {
  return (
    <Container>
      <CardLayout>{children}</CardLayout>
    </Container>
  );
};

export default Card;
