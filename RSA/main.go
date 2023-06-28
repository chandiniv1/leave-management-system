package main

import (
	"fmt"
	"math"
)

func gcd(e, phi int) int {
	if phi != 0 {
		return gcd(phi, e%phi)
	}
	return e

}

func encryption(msg float64, publickey int, n int) float64 {
	t := math.Pow(msg, float64(publickey))
	c := int(t) % n
	return float64(c)
}

func decryption(c float64, privatekey int, n int) float64 {
	t := math.Pow(c, float64(privatekey))
	msg := int(t) % n
	return float64(msg)
}
func main() {
	p := 3
	q := 7
	n := p * q
	phi := (p - 1) * (q - 1)
	e := 2
	// d:=((k*phi)+1)/e
	for e < phi {
		//e must be smaller than phi and coprime to phi
		if gcd(e, phi) == 1 {
			break
		} else {
			e = e + 1
		}
	}
	publickey := e

	//generating private key
	k := 2 //some constant
	privatekey := (1 + (k * phi)) / e
	msg := 12.0
	fmt.Println("msg data: ", msg)
	c := encryption(msg, publickey, n)
	fmt.Println("encrypted msg: ", c)
	fmt.Println("decrypted msg: ", decryption(c, privatekey, n))
}