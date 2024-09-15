# Katalon API Automation Testing

This API automation test is built using Katalon and integrates seamlessly with the KasirAja API

#### A. API Methods Utilized
1. POST (Create)
2. GET (Retrieve)
3. PUT (Update)
4. DELETE (Remove)

#### B. Available Endpoints
1. Categories: Includes methods for creating, updating, and deleting categories.
2. Products: Includes methods for creating and retrieving product details.

#### C.Test Cases
1. POST Add Category – Adds a new category.
2. PUT Update Category – Updates an existing category's information.
3. DELETE Category – Deletes a category.
4. POST Add Product – Adds a new product.
5. GET Product – Retrieves details of an existing product.

#### D. Test Suite
1. Regression Test Suite: Categories
2. Regression Test Suite: Products

#### E. Steps to Execute the Automation in Katalon:
1. Open Katalon and select New Project.
2. Click Clone Git and paste the repository's HTTPS link
3. Navigate to the Object Repository.
4. Open the Auth folder and run the Registration and Login test cases.
5. After logging in, obtain the access token from the response and assign it to the global variable accessToken.
6. Once the token is set, open the Test Case folder.
7. Select the appropriate test cases and click Run.
