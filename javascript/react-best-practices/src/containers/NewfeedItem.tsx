import React from "react";
import styled from "styled-components";
import Card from "../components/Card";

interface Props {
  title?: string;
  body?: string;
}

const NewsfeedItem = ({
  children,
  ...props
}: React.PropsWithChildren<Props>) => {
  let { title, body } = props;
  return (
    <Card>
      <h1>{title}</h1>
      <p>{body || ""}</p>
    </Card>
  );
};

export default NewsfeedItem;
