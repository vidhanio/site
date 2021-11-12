import type { NextPage } from "next";
import Head from "next/head";
import Image from "next/image";
import React from "react";

interface BoxProps {
  coords: [number, number];
  turn: "X" | "O";
  onFinishTurn: (coords: [number, number]) => void;
  winner: "X" | "O" | null | undefined;
}

interface SmallTTTState {
  turn: "X" | "O";
  board: ("X" | "O" | null | undefined)[][];
  winner: "X" | "O" | null | undefined;
}

class Box extends React.Component<BoxProps> {
  constructor(props: BoxProps) {
    super(props);
    this.handleFinishTurn = this.handleFinishTurn.bind(this);
  }

  handleFinishTurn() {
    this.props.onFinishTurn(this.props.coords);
  }

  render() {
    return (
      <button
        className={`font-mono text-xl transition-colors ${
          this.props.winner !== undefined ? "text-red-400" : "text-blue-400"
        }`}
        onClick={
          this.props.winner === undefined
            ? this.handleFinishTurn
            : () => undefined
        }
      >
        {this.props.winner === undefined
          ? "."
          : this.props.winner === null
          ? "T"
          : this.props.winner}
      </button>
    );
  }
}

class SmallTTT extends React.Component<{}, SmallTTTState> {
  constructor(props: {}) {
    super(props);
    this.state = {
      board: [
        [undefined, undefined, undefined],
        [undefined, undefined, undefined],
        [undefined, undefined, undefined],
      ],
      winner: undefined,
      turn: "X",
    };

    this.toggleTurn = this.toggleTurn.bind(this);
    this.finishTurn = this.finishTurn.bind(this);
  }

  toggleTurn() {
    this.setState({ turn: this.state.turn === "X" ? "O" : "X" });
  }

  finishTurn(coords: [number, number]) {
    const newBoard = this.state.board;
    newBoard[coords[1]][coords[0]] = this.state.turn;
    this.setState({ board: newBoard });
    this.toggleTurn();
    this.checkWinner();
  }

  setWinner(winner: "X" | "O" | null | undefined) {
    this.setState({ winner: winner }, () => {
      if (this.state.winner !== undefined) {
        const winner = this.state.winner;
        this.setState({
          board: [
            [winner, winner, winner],
            [winner, winner, winner],
            [winner, winner, winner],
          ],
        });
      }
    });
  }

  checkWinner() {
    const board = this.state.board;
    for (let i = 0; i < 3; i++) {
      if (
        board[i][0] != undefined &&
        board[i][0] === board[i][1] &&
        board[i][0] === board[i][2]
      ) {
        this.setWinner(board[i][0]);
        return;
      }
    }
    for (let i = 0; i < 3; i++) {
      if (
        board[0][i] != undefined &&
        board[0][i] === board[1][i] &&
        board[0][i] === board[2][i]
      ) {
        this.setWinner(board[0][i]);
        return;
      }
    }
    if (
      board[0][0] != undefined &&
      board[0][0] === board[1][1] &&
      board[0][0] === board[2][2]
    ) {
      this.setWinner(board[0][0]);
      return;
    }
    if (
      board[0][2] != undefined &&
      board[0][2] === board[1][1] &&
      board[0][2] === board[2][0]
    ) {
      this.setWinner(board[0][2]);
      return;
    }
    if (board.flat().filter((x) => x === undefined).length === 0) {
      this.setWinner(null);
    }
  }

  renderBox(coords: [number, number]) {
    return (
      <Box
        onFinishTurn={this.finishTurn}
        coords={coords}
        turn={this.state.turn}
        winner={this.state.board[coords[1]][coords[0]]}
      />
    );
  }

  render() {
    return (
      <div className="flex flex-col">
        <div className="flex flex-row">
          {this.renderBox([0, 0])}
          {this.renderBox([1, 0])}
          {this.renderBox([2, 0])}
        </div>
        <div className="flex flex-row">
          {this.renderBox([0, 1])}
          {this.renderBox([1, 1])}
          {this.renderBox([2, 1])}
        </div>
        <div className="flex flex-row">
          {this.renderBox([0, 2])}
          {this.renderBox([1, 2])}
          {this.renderBox([2, 2])}
        </div>
      </div>
    );
  }
}

class MediumTicTacToe extends React.Component<{}, {}> {
  constructor(props: {}) {
    super(props);
  }

  render() {
    return (
      <div className="flex flex-col gap-4">
        <div className="flex flex-row gap-4">
          <SmallTTT />
          <SmallTTT />
          <SmallTTT />
        </div>
        <div className="flex flex-row gap-4">
          <SmallTTT />
          <SmallTTT />
          <SmallTTT />
        </div>
        <div className="flex flex-row gap-4">
          <SmallTTT />
          <SmallTTT />
          <SmallTTT />
        </div>
      </div>
    );
  }
}

export default MediumTicTacToe;
