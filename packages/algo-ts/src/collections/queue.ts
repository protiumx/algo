/** 
* Simple generic Queue implementation using JavaScript Array
*/
export default class Queue<T> {
  private data: Array<T> = [];

  enqueue(element: T) {
    this.data.push(element);
  }

  dequeue(): T | null {
    if (this.isEmpty()) {
      return null;
    }
    return this.data.shift()!;
  }

  isEmpty(): boolean {
    return this.data.length === 0;
  }
}
