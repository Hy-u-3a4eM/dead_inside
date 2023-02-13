from time import sleep

def main() -> None:
    num1: int = 1000
    num2: int = 7
    while num2 <= num1:
        print("{} - {}? ".format(num1, num2), end='', flush=True)

        num1 -= num2

        print(num1)

        sleep(0.035)

if __name__ == "__main__":
    main()

