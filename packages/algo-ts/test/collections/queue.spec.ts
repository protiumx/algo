import Queue from "$/collections/queue";

describe("Queue", () => {
  it("should enqueue and dequeue correctly", () => {
    const q = new Queue<string>();
    q.enqueue("test");
    q.enqueue("1");
    expect(q.dequeue()).toEqual("test");
  });

  it("should return `null` if queue is empty", () => {
    const q = new Queue();
    expect(q.isEmpty()).toBeTruthy();
    expect(q.dequeue()).toBeNull();
  });
});
