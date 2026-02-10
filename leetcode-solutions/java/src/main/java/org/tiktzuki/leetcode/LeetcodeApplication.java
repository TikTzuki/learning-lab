package org.tiktzuki.leetcode;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
public class LeetcodeApplication {

    public static void main(String[] args) {
        int z = 2;
        z |= 1 << 1;
        System.out.println(z);
//        SpringApplication.run(LeetcodeApplication.class, args);
    }

}
