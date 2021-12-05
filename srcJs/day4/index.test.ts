function part1(path: string): number {
  console.log(path)
  return 0
}

describe('Day 4', () => {
  test('Validate Part1', async () => {
    expect(part1('./srcJs/test.txt')).toBe(0)
  })
})
