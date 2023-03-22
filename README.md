# Project 3 - Passport photo processor using AWS Lambda and AWS S3

## Overview 
My project is meant to simulate a service for a business that processes passport photos. It uses an AWS Lambda function that is triggered when any .jpg files are added to the source S3 bucket, which is meant to contain all the original passport photos that are taken by the business. The lambda function will take the new picture, resize it to the standard passport photo size and add a watermark to the picture. This processed picture will be uploaded to a different S3 bucket, which is meant to store the final versions. Then, the 'business' can distribute those pictures to the customers before they decide to purchase. 

## Setup
In order to run this application, Rust and cargo lambda will need to be installed - installation guides for the latter can be found in the first link in the References section. 


### Configure AWS
1. Create an IAM User policy for "LambdaDeveloper" with `AWSLambda_FullAccess` permissions and added custom inline permission config
```
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Sid": "PermissionsToDeploy",
            "Effect": "Allow",
            "Action": [
                "iam:CreateRole",
                "iam:CreatePolicy",
                "iam:PutRolePolicy",
                "iam:AttachRolePolicy",
                "iam:UpdateAssumeRolePolicy"
            ],
            "Resource": "*"
        }
    ]
}
```

2. Add "LambdaDeveloper" keys to your local ~/.aws/credentials file with environment variables: 
* aws_access_key_id
* aws_secret_access_key
* aws_role_arn
* region 

3. Create an IAM Role policy for "S3xLambdaAccessRole" with `AmazonS3FullAccess` and `AWSLambdaBasicExecutionRole` permissions
4. Allocate 2 x S3 Buckets (`passport-originals` and `passport-watermarking`) and add an Access Point to each


### Create Lambda Function 
This is done in the AWS Lambda console. A function is made called `watermark` with the following settings:
- Runtime = Custom runtime on Amazon Linux 2
- Handler function = handler
- Architecture = arm64

Then, add the "S3xLambdaAccessRole" created to the Execution role of the Lambda function permissions. 


### Build and Deploy Lambda Function 
- Build and store output in .zip file 
```cargo lambda build --release --arm64 --output-format zip```
- Upload `bootstrap.zip` stored in `target/lambda/watermark/bootstrap.zip` locally and upload to the Lambda function code 


## Architecture 

# ADD PIC 

## Benchmarking 
- Building the release occurs in 1 minute
- Each trigger of the lambda function takes about ~700 ms

# ADD PICS



## References
- [Cargo Lambda](https://www.cargo-lambda.info/guide/getting-started.html)
- [aws-sdk-rust S3 Examples](https://github.com/awslabs/aws-sdk-rust/tree/main/examples/s3)
- [AWS Lambda in Rust Tutorial](https://www.youtube.com/watch?v=PmtwtK6jyLc)